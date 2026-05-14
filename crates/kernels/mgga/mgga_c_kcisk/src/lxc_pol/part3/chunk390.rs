//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 390/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk390<F: Float>(t2864: F, t2867: F, t2869: F, t2873: F, t2875: F, t2877: F, t830: F, t815: F, t813: F, t14: F, t31: F, t2857: F, t119: F, t56: F, t69: F, t45: F, t5: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2879 = -0.42198333333333333333e0 * t2864 + 0.84396666666666666666e0 * t2867 + 0.39862222222222222223e0 * t2869 + 0.68258333333333333333e-1 * t2873 + 0.13651666666666666667e0 * t2875 + 0.13692777777777777778e0 * t2877;
    let t2880 = t2879 * t830;
    let t2882 = 1.0 * t815 * t2880;
    let t2883 = t813 * t813;
    let t2884 = 1.0 / t2883;
    let t2885 = t14 * t2884;
    let t2886 = t31 * t31;
    let t2887 = 1.0 / t2886;
    let t2888 = t2857 * t2887;
    let t2890 = 0.16081824322151104822e2 * t2885 * t2888;
    let t2892 = t69 * t119 * t56;
    let t2895 = t45 * t5;
    (t2879, t2880, t2882, t2883, t2884, t2885, t2886, t2887, t2888, t2890, t2892, t2895)
}
