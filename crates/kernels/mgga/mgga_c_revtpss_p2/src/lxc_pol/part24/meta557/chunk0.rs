//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1665/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1665<F: Float>(t23535: F, t4598: F, t18987: F, t6120: F, t4614: F, t18979: F, t11341: F, t141: F, t88116: F, t88095: F, t930: F, t77804: F, t88085: F, t88093: F, t88104: F, t88108: F, t88114: F, t88122: F, t88130: F) -> (F, F, F, F, F, F, F) {
    let t88220 = t4598 * t23535;
    let t88222 = t18987 * t6120;
    let t88224 = t4614 * t23535;
    let t88226 = t18979 * t6120;
    let t88229 = t141 * t11341 * t88116;
    let t88232 = t141 * t930 * t88095;
    let t88242 = -F::new(0.51785e1) * t88220 - F::cast_from(0.247573125e0_f64) * t88222 + F::new(0.3300975e0) * t88224 + F::new(0.11651625e2) * t88226 - F::new(0.22076e0) * t88229 + F::new(0.66228e0) * t88232 + F::new(0.72462e1) * t88085 + F::new(0.181155e1) * t88093 - F::cast_from(0.89459259259259259259e0_f64) * t88104 - F::new(0.301925e0) * t88108 + F::cast_from(0.40256666666666666666e1_f64) * t88114 - F::new(0.72462e1) * t88122 - F::cast_from(0.60384999999999999999e0_f64) * t88130 - F::new(0.132456e1) * t77804;
    (t88220, t88222, t88224, t88226, t88229, t88232, t88242)
}
