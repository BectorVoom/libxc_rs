//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1000/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1000<F: Float>(t2075: F, t6006: F, t18984: F, t3482: F, t1056: F, t2152: F, t5675: F, t19813: F, t5606: F, t6007: F, t1411: F, t3508: F, t8078: F, t13956: F, t14160: F, t19791: F, t19807: F, t19808: F, t19810: F, t19833: F, t19837: F, t19847: F, t19857: F, t26851: F, t26858: F, t26862: F, t26867: F, t26869: F) -> (F, F, F, F, F, F, F, F) {
    let t26871 = t2075 * t6006;
    let t26872 = t18984 * t26871;
    let t26873 = t3482 * t26872;
    let t26875 = t2152 * t1056;
    let t26876 = t5675 * t26875;
    let t26877 = t19813 * t26876;
    let t26878 = t3482 * t26877;
    let t26881 = t5606 * t6007;
    let t26882 = t1411 * t26881;
    let t26884 = t3508 * t8078;
    let t26885 = t1411 * t26884;
    let t26887 = -0.16581944444444444444e-1 * t26851 - t19791 + t19807 - 0.22109259259259259259e-2 * t19808 + 0.22109259259259259259e-2 * t19810 - 0.36848765432098765431e-3 * t13956 - 0.27636574074074074073e-2 * t26858 - 0.16581944444444444444e-2 * t26862 - t19833 + 0.22109259259259259259e-2 * t19837 - t19847 + t19857 - 0.44218518518518518517e-2 * t26867 - 0.33163888888888888888e-2 * t26869 - 0.7369753086419753086e-3 * t26873 - 0.44218518518518518516e-2 * t26878 - 0.55273148148148148147e-3 * t14160 - 0.11054629629629629629e-2 * t26882 - 0.33163888888888888888e-2 * t26885;
    (t26871, t26873, t26875, t26876, t26878, t26882, t26885, t26887)
}
