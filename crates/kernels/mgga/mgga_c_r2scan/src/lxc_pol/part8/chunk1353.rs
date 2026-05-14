//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1353/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1353<F: Float>(t113: F, t32516: F, t2115: F, t2155: F, t26088: F, t9297: F, t10140: F, t1632: F, t551: F, t6362: F, t19863: F, t19886: F, t19904: F, t24523: F, t24547: F, t24575: F, t24583: F, t24707: F, t24711: F, t24712: F, t24718: F) -> (F, F) {
    let t33074 = t32516 * t113;
    let t33075 = t2115 * t33074;
    let t33076 = t2155 * t33075;
    let t33078 = t26088 * t9297;
    let t33082 = t6362 * t551 * t1632 * t10140;
    let t33084 = 0.28914548798370980346e-4 * t19863 - 0.1713958891116262235e0 * t19886 + t19904 + t24523 + t24547 + t24575 + t24583 - 0.4075278042773769234e0 * t24707 - t24711 + 0.9878173774403267398e-1 * t24712 - 0.523649308946876022e0 * t24718 - 0.48787202696913915093e-2 * t33076 + 0.34930954652346593433e-1 * t33078 + 0.69345773920434148507e0 * t33082;
    (t33075, t33084)
}
