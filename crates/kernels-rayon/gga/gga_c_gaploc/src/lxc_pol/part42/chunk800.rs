//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 800/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk800(t10736: f64, t28412: f64, t913: f64, t2365: f64, t33087: f64, t8775: f64, t10639: f64, t10912: f64, t787: f64, t899: f64, t13118: f64, t15362: f64) -> (f64, f64, f64, f64) {
    let t43432 = t28412 * t913 * t10736;
    let t43446 = t8775 * t2365 * t33087;
    let t43454 = t787 * t10912 * t899 * t913 * t10639;
    let t43464 = t15362 * t13118;
    (t43432, t43446, t43454, t43464)
}
