//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1233/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1233<F: Float>(t26622: F, t5210: F, t7824: F, t5964: F, t1693: F, t2483: F, t22235: F, t1419: F, t2794: F, t5300: F, t963: F, t5286: F, t2768: F, t5251: F, t5250: F, t2482: F, t5249: F, t5252: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26623 = 0.57791679765211885292e1 * t26622;
    let t26625 = t7824 * t5210;
    let t26627 = t7824 * t5964;
    let t26629 = t2483 * t1693;
    let t26630 = 0.3903689268108626343e0 * t26629;
    let t26638 = 48.0 * t22235;
    let t26641 = t1419 * t2794;
    let t26642 = 36.0 * t26641;
    let t26667 = t963 * t5300;
    let t26669 = t963 * t5286;
    let t26671 = t2768 * t5251;
    let t26672 = t5250 * t26671;
    let t26673 = 0.12154685976e1 * t26672;
    let t26675 = t5249 * t2482 * t5252;
    (t26623, t26625, t26627, t26630, t26638, t26642, t26667, t26669, t26671, t26673, t26675)
}
