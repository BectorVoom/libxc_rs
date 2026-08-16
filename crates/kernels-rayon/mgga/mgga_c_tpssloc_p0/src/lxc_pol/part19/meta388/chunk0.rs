//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1457/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1457(t3502: f64, t42341: f64, t44696: f64, t23508: f64, t3508: f64, t225: f64, t44657: f64, t1209: f64, t475: f64, t43670: f64, t43672: f64, t43674: f64, t43678: f64, t43683: f64, t43685: f64, t43687: f64, t43695: f64, t43702: f64, t43915: f64, t43924: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44753 = t44696 * t42341 * t3502;
    let t44754 = t23508 * t3508;
    let t44774 = t44657 * t225;
    let t44785 = t44696 * t42341 * t1209;
    let t44786 = t23508 * t475;
    let t44792 = -t43670 - t43672 + t43674 - t43678 - t43683 + t43685 - t43687 - t43695 - t43702 - t43915 + t43924;
    (t44753, t44754, t44774, t44785, t44786, t44792)
}
