//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 978/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk978<F: Float>(t13377: F, t19822: F, t3482: F, t3778: F, t5606: F, t1411: F, t3739: F, t5988: F, t3508: F, t5975: F, t5985: F, t3512: F, t5981: F, t13956: F, t13960: F, t13962: F, t19816: F, t19820: F) -> (F, F, F, F, F, F, F) {
    let t19823 = t13377 * t19822;
    let t19824 = t3482 * t19823;
    let t19829 = t5606 * t3778;
    let t19830 = t1411 * t19829;
    let t19832 = t3739 * t5988;
    let t19833 = 0.33163888888888888888e-2 * t19832;
    let t19834 = t3508 * t5975;
    let t19835 = t1411 * t19834;
    let t19837 = t3739 * t5985;
    let t19839 = t3512 * t5981;
    let t19840 = t1411 * t19839;
    let t19842 = -0.44218518518518518517e-2 * t19816 + 0.3684876543209876543e-2 * t19820 - 0.22109259259259259258e-2 * t19824 - 0.73697530864197530861e-3 * t13956 + 0.14739506172839506172e-2 * t13960 + 0.16581944444444444444e-2 * t13962 - 0.55273148148148148147e-3 * t19830 - t19833 + 0.13265555555555555555e-1 * t19835 + 0.22109259259259259258e-2 * t19837 - 0.88437037037037037034e-2 * t19840;
    (t19824, t19830, t19832, t19835, t19837, t19840, t19842)
}
