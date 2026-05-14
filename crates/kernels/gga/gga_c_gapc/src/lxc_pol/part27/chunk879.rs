//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 879/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk879<F: Float>(t11876: F, t7949: F, t959: F, t11875: F, t2767: F, t3717: F, t7294: F, t11365: F, t2660: F, t7880: F, t612: F, t7953: F, t291: F, t7956: F, t9066: F, t3363: F, t3687: F) -> (F, F, F, F, F, F, F, F) {
    let t11878 = t11876 * t959 * t7949;
    let t11879 = t11875 * t11878;
    let t11882 = t7294 * t3717 * t2767;
    let t11885 = t2660 * t11365 * t7880;
    let t11887 = t7953 * t612;
    let t11889 = t9066 * t291 * t7956;
    let t11890 = t11887 * t11889;
    let t11892 = t3363 * t3687;
    (t11878, t11879, t11882, t11885, t11887, t11889, t11890, t11892)
}
