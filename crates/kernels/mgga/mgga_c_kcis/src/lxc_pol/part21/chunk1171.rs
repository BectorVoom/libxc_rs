//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1171/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1171<F: Float>(t28280: F, t28294: F, t11230: F, t1282: F, t1291: F, t15692: F, t1872: F, t27100: F, t27991: F, t27993: F, t27995: F, t27996: F, t28004: F, t28007: F, t28011: F, t28072: F, t28260: F, t28265: F, t3664: F, t5360: F, t7812: F, t7823: F, t8117: F) -> (F, F) {
    let t28295 = t28280 + t28294;
    let t28297 = -F::new(6.0) * t11230 * t28260 - t1282 * t28295 - t1291 * t28265 + F::new(2.0) * t15692 * t7812 - t1872 * t27100 - t3664 * t8117 - t5360 * t7823 - t27991 + t27993 - t27995 + t27996 - t28004 - t28007 - t28011 + t28072;
    (t28295, t28297)
}
