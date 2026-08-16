//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 568/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk568(t10023: f64, t10024: f64, t10001: f64, t10006: f64, t10010: f64, t10015: f64, t10019: f64, t10022: f64, t5748: f64, t813: f64, t833: f64, t9982: f64, t9986: f64, t9990: f64, t9993: f64, t9997: f64) -> (f64, f64) {
    let t10026 = 0.89376224879626066674e-1_f64 * t10023 * t10024;
    let t10027 = 0.31952438294933958064e-1_f64 * t9982 + 0.27606906686822939767e2_f64 * t5748 * t9986 - 0.46011511144704899612e1_f64 * t813 * t9990 + 0.11502877786176224903e2_f64 * t833 * t9993 - 0.92023022289409799224e1_f64 * t813 * t9997 + 0.43710935587469654631e2_f64 * t833 * t10001 + t10006 - 0.15976219147466979032e-1_f64 * t10010 + 0.15976219147466979032e-1_f64 * t10015 + 0.7988109573733489516e-2_f64 * t10019 - t10022 - t10026;
    (t10026, t10027)
}
