//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 970/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk970<F: Float>(t10740: F, t776: F, t2080: F, t3344: F, t1050: F, t120: F, t20621: F, t10698: F, t10701: F, t2090: F, t3294: F, t3296: F, t2096: F, t2167: F, t546: F, t10729: F) -> (F, F, F, F, F, F, F, F, F) {
    let t37903 = t776 * t10740;
    let t37905 = t2080 * t3344;
    let t37919 = t120 * t20621 * t1050;
    let t37925 = t10698 * t10701;
    let t37932 = t120 * t2090 * t3294;
    let t37933 = t37932 * t3296;
    let t37935 = t2167 * t2096;
    let t37936 = t546 * t37935;
    let t37937 = t37936 * t10729;
    (t37903, t37905, t37919, t37925, t37932, t37933, t37935, t37936, t37937)
}
