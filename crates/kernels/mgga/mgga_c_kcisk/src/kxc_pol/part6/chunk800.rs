//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 800/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk800<F: Float>(t28855: F, t4629: F, t11393: F, t28591: F, t706: F, t11402: F, t22417: F, t2487: F, t1887: F, t28571: F, t1882: F, t28393: F, t11400: F, t1421: F, t16879: F, t22469: F, t22512: F, t22524: F, t28532: F, t28852: F, t456: F, t604: F) -> (F, F, F, F) {
    let t28856 = t4629 * t28855;
    let t28859 = t11393 * t28591;
    let t28860 = t706 * t28859;
    let t28865 = t11402 * t22417 * t2487;
    let t28868 = t1887 * t28571;
    let t28869 = t706 * t28868;
    let t28873 = t1882 * t28393;
    let t28874 = t706 * t28873;
    let t28881 = -0.4435040025e-2 * t1421 * t28852 + 0.887008005e-2 * t1421 * t28856 - 0.59133867e-2 * t456 * t28860 + 0.1478346675e-2 * t22469 - 0.59133867e-2 * t11400 * t28865 - 0.98556445e-3 * t456 * t28869 - 0.19711289e-2 * t22512 + 0.1478346675e-2 * t456 * t28874 + 0.295669335e-2 * t22524 - 4.0 * t604 * t28532 + 0.65704296666666666665e-3 * t16879;
    (t28859, t28868, t28873, t28881)
}
