//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 631/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk631<F: Float>(t4924: F, t587: F, t1791: F, t642: F, t1793: F, t626: F, t422: F, t639: F, t4872: F, t4873: F, t4876: F, t4881: F, t4885: F, t4890: F, t4895: F, t4900: F, t4905: F, t4907: F, t4910: F, t4912: F, t4915: F, t4917: F, t4922: F) -> (F, F, F, F, F, F, F) {
    let t4926 = F::new(8.0) / F::new(15.0) * t587 * t4924;
    let t4927 = t642 * t1791;
    let t4928 = t1793 * t626;
    let t4929 = t4928 * t422;
    let t4930 = t4927 * t4929;
    let t4932 = F::new(8.0) / F::new(15.0) * t639 * t4930;
    let t4933 = -t4872 + F::new(0.9973633333333333333e-1) * t4873 + t4876 - t4881 + t4885 + t4890 - t4895 - t4900 + t4905 + t4907 + t4910 + t4912 + t4915 - t4917 - t4922 + t4926 + t4932;
    (t4926, t4927, t4928, t4929, t4930, t4932, t4933)
}
