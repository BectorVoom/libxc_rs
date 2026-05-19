//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 568/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk568<F: Float>(t10023: F, t10024: F, t10001: F, t10006: F, t10010: F, t10015: F, t10019: F, t10022: F, t5748: F, t813: F, t833: F, t9982: F, t9986: F, t9990: F, t9993: F, t9997: F) -> (F, F) {
    let t10026 = F::cast_from(0.89376224879626066674e-1_f64) * t10023 * t10024;
    let t10027 = F::cast_from(0.31952438294933958064e-1_f64) * t9982 + F::cast_from(0.27606906686822939767e2_f64) * t5748 * t9986 - F::cast_from(0.46011511144704899612e1_f64) * t813 * t9990 + F::cast_from(0.11502877786176224903e2_f64) * t833 * t9993 - F::cast_from(0.92023022289409799224e1_f64) * t813 * t9997 + F::cast_from(0.43710935587469654631e2_f64) * t833 * t10001 + t10006 - F::cast_from(0.15976219147466979032e-1_f64) * t10010 + F::cast_from(0.15976219147466979032e-1_f64) * t10015 + F::cast_from(0.7988109573733489516e-2_f64) * t10019 - t10022 - t10026;
    (t10026, t10027)
}
