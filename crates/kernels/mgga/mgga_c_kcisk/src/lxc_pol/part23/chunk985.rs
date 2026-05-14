//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 985/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk985<F: Float>(t1411: F, t19935: F, t1451: F, t19861: F, t3503: F, t5606: F, t13383: F, t2232: F, t13955: F, t2178: F, t1413: F, t5866: F, t1441: F, t3733: F, t5886: F, t12817: F, t2236: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19936 = t1411 * t19935;
    let t19938 = t19861 * t1451;
    let t19939 = t1411 * t19938;
    let t19941 = t5606 * t3503;
    let t19942 = t1411 * t19941;
    let t19944 = t13383 * t2232;
    let t19945 = t1411 * t19944;
    let t19948 = t13955 * t2178;
    let t19950 = t5866 * t1413;
    let t19951 = t19950 * sigma0;
    let t19952 = t19951 * t1441;
    let t19953 = t1411 * t19952;
    let t19955 = t5886 * t3733;
    let t19956 = t1411 * t19955;
    let t19958 = t12817 * t2236;
    (t19936, t19939, t19942, t19945, t19948, t19950, t19951, t19953, t19956, t19958)
}
