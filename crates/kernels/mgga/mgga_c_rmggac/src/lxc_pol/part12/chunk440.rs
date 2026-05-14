//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 440/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk440<F: Float>(t1653: F, t333: F, t1598: F, t866: F, t571: F, t833: F, t325: F, t623: F, t4698: F, t4700: F, t4697: F, t4705: F, t1664: F, t356: F, t552: F, t848: F) -> (F, F, F, F, F, F, F) {
    let t4974 = t1653 * t333;
    let t4977 = t1598 * t866;
    let t4982 = t571 * t833;
    let t4985 = t623 * t325;
    let t4997 = 1584.0 * t4698;
    let t4998 = 1872.0 * t4700;
    let t4999 = t4697 - t4997 - t4998 + t4705;
    let t5002 = t1664 * t356;
    let t5005 = t552 * t848;
    (t4974, t4977, t4982, t4985, t4999, t5002, t5005)
}
