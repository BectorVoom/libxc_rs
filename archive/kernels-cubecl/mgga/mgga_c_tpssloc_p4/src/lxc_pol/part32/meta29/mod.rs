//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta29 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk214;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk215;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk216;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk217;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk218;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk219;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk220;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk221;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta29<F: Float>(t544: F, t554: F, t532: F, t242: F, t248: F, t219: F, t541: F, t539: F, t553: F, t254: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t555, t556) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk214::<F>(t544, t554, t532);
        let t557 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk215::<F>(t556);
        let t559 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk216::<F>(t242, t248, t557);
        let t562 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk217::<F>(t219, t541, t555, t559);
        let (t563, t564) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk218::<F>(t539, t562, t553);
        let (t566, t567) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk219::<F>(t544, t564);
        let t568 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk220::<F>(t254, t567);
        let (t570, t571) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk221::<F>(t563, t568);
    (t555, t556, t557, t559, t562, t563, t564, t566, t567, t568, t570, t571)
}
