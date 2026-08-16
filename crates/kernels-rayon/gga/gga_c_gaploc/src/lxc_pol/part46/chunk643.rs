//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 643/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk643(t10820: f64, t7573: f64, t7427: f64, t2013: f64, t3489: f64, t123: f64, t2925: f64, t883: f64, t969: f64, t825: f64, t10678: f64, t549: f64) -> (f64, f64, f64, f64, f64) {
    let t10821 = t7573 * t10820;
    let t10823 = 0.62115540045351614476e2_f64 * t7427 * t10821;
    let t10824 = t2013 * t3489;
    let t10825 = 0.19171462976960374838e0_f64 * t10824;
    let t10826 = t2925 * t123;
    let t10827 = t10826 * t883;
    let t10828 = t969 * t10827;
    let t10829 = t825 * t10828;
    let t10830 = 0.19171462976960374838e0_f64 * t10829;
    let t10831 = t549 * t10678;
    (t10823, t10825, t10827, t10830, t10831)
}
