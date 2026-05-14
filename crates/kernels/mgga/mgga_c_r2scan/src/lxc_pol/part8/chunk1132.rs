//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1132/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1132<F: Float>(t1691: F, t1813: F, t1835: F, t61: F, t424: F, t5435: F, t704: F, t5961: F, t5967: F, t1762: F, t1763: F, t5798: F, t5527: F, t5960: F, t518: F, t706: F) -> (F, F, F, F, F, F, F) {
    let t21311 = t1813 * t1691;
    let t21313 = 0.69350015718254262349e2 * t61 * t1835 * t21311;
    let t21315 = t424 * t704 * t5435;
    let t21326 = t5967 * t5961;
    let t21330 = 0.1301229756036208781e0 * t1762 * t1763 * t5798;
    let t21333 = 0.11558335953042377059e2 * t1762 * t5960 * t5527;
    let t21340 = 0.13494234507042165137e0 * t1762 * t518 * t704 * t706;
    (t21311, t21313, t21315, t21326, t21330, t21333, t21340)
}
