//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1317/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1317<F: Float>(t96217: F, t27811: F, t61287: F, t4981: F, t982: F, t990: F, t26757: F, t27832: F, t26748: F, t27911: F, t7706: F, t93087: F, t93425: F, t95917: F, t95923: F, t96204: F, t96207: F, t96212: F, t96215: F) -> F {
    let t96218 = F::new(0.22109259259259259258e-2) * t96217;
    let t96221 = t27811 * t61287;
    let t96227 = t4981 * t982 * t990;
    let t96231 = F::new(0.15445601851851851852e-3) * t27832 * t26757;
    let t96232 = F::new(0.29479012345679012345e-2) * t96204 - F::new(0.11054629629629629629e-2) * t96207 - F::new(0.16581944444444444444e-2) * t93087 - F::new(0.49745833333333333332e-2) * t96212 + F::new(0.33163888888888888888e-2) * t96215 + t96218 - F::new(0.61836467013888888888e-4) * t93425 * t95917 - F::new(0.12378114784505208333e-4) * t96221 * t95923 - F::new(0.13901041666666666667e-2) * t26748 * t27911 + F::new(0.12356481481481481482e-2) * t96227 * t7706 - t96231;
    t96232
}
