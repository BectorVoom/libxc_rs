//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 973/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk973<F: Float>(t1887: F, t22632: F, t706: F, t11344: F, t11350: F, t11400: F, t1421: F, t16844: F, t16863: F, t16865: F, t16879: F, t16885: F, t1689: F, t16897: F, t16900: F, t16902: F, t16941: F, t22512: F, t22515: F, t22518: F, t22521: F, t22524: F, t22527: F, t22531: F, t22535: F, t456: F, t8616: F) -> (F, F) {
    let t22633 = t1887 * t22632;
    let t22634 = t706 * t22633;
    let t22641 = -0.65704296666666666667e-3 * t22512 + 0.26281718666666666666e-2 * t11400 * t22515 - 0.21901432222222222222e-2 * t16844 * t22518 - 0.19711289e-2 * t11400 * t22521 + 0.98556445e-3 * t22524 + 0.1478346675e-2 * t1421 * t22527 - 0.295669335e-2 * t1421 * t22531 - 0.295669335e-2 * t1421 * t22535 - t16863 + t16865 + 0.21901432222222222222e-3 * t11344 - 0.32852148333333333333e-3 * t11350 - 4.0 * t1689 * t8616 - 0.98556445e-3 * t456 * t22634 + 0.43802864444444444443e-3 * t16879 - 0.65704296666666666667e-3 * t16885 - 0.2920190962962962963e-3 * t16897 + t16900 - t16902 - 0.17521145777777777778e-2 * t16941;
    (t22633, t22641)
}
