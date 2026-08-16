//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1298/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1298(t3540: f64, t8043: f64, t27628: f64, t27634: f64, t1409: f64, t461: f64, t1009: f64, t7324: f64, t3545: f64, t8020: f64, t15753: f64, t7310: f64) -> (f64, f64, f64, f64, f64) {
    let t95365 = t8043 * t3540;
    let t95387 = t27634 * t27628;
    let t95420 = t1409 * t461;
    let t95422 = t7324 * t95420 * t1009;
    let t95450 = t8020 * t3545;
    let t95512 = t7310 * t15753;
    (t95365, t95387, t95422, t95450, t95512)
}
