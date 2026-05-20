//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3269/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3269<F: Float>(t74130: F, t74132: F, t48313: F, t47110: F, t189: F, t512: F, t85970: F, t22789: F, t749: F, t47119: F, t40067: F, t40072: F, t47109: F, t47113: F, t47116: F, t47118: F, t47122: F, t47124: F, t48312: F) -> (F, F, F, F, F, F, F, F) {
    let t85979 = F::cast_from(0.35089341735807877242e1_f64) * t74130;
    let t85980 = F::new(24.0) * t74132;
    let t85981 = F::cast_from(0.65061487801810439052e-1_f64) * t48313;
    let t85982 = F::cast_from(0.35089341735807877242e1_f64) * t47110;
    let t85984 = t512 * t85970 * t189;
    let t85986 = t512 * t22789 * t749;
    let t85987 = F::cast_from(0.32530743900905219526e-1_f64) * t47119;
    let t85988 = t85979 - t85980 + t48312 - t85981 + t40067 - t40072 - t47109 - t85982 + t85984 + t85986 + t47113 + t47116 - t47118 - t85987 + t47122 + t47124;
    (t85979, t85980, t85981, t85982, t85984, t85986, t85987, t85988)
}
