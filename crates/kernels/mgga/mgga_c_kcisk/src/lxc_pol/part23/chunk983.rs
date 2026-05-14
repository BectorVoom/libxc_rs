//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 983/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk983<F: Float>(t19904: F, t3776: F, t1340: F, t1411: F, t12817: F, t2177: F, t1339: F, t13401: F, t2075: F, t14160: F, t14162: F, t19847: F, t19851: F, t19854: F, t19857: F, t19859: F, t19863: F, t19866: F, t19871: F, t19876: F, t19879: F, t19884: F, t19889: F, t19893: F, t19898: F, t19902: F) -> (F, F, F, F) {
    let t19905 = t3776 * t19904;
    let t19906 = t1340 * t19905;
    let t19907 = t1411 * t19906;
    let t19909 = t12817 * t2177;
    let t19910 = t1339 * t19909;
    let t19912 = t13401 * t2075;
    let t19913 = t1340 * t19912;
    let t19914 = t1339 * t19913;
    let t19916 = -t19847 + 0.49745833333333333332e-2 * t19851 + 0.13265555555555555555e-1 * t19854 + t19857 + 0.66327777777777777776e-2 * t19859 + 0.33163888888888888888e-2 * t19863 + 0.16581944444444444444e-2 * t19866 - 0.11054629629629629629e-2 * t14160 + 0.1621345679012345679e-1 * t14162 - 0.33163888888888888888e-2 * t19871 + 0.99491666666666666664e-2 * t19876 - 0.88437037037037037034e-2 * t19879 + 0.73697530864197530862e-3 * t19884 + 0.33163888888888888888e-2 * t19889 + 0.88437037037037037034e-2 * t19893 + 0.55273148148148148147e-3 * t19898 + 0.29479012345679012345e-2 * t19902 - 0.55273148148148148147e-3 * t19907 + 0.16581944444444444444e-2 * t19910 + 0.1621345679012345679e-1 * t19914;
    (t19907, t19910, t19914, t19916)
}
