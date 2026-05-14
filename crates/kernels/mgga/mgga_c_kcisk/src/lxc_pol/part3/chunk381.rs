//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 381/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk381<F: Float>(t772: F, t695: F, t786: F, t1060: F, t1775: F, t785: F, t657: F, t1990: F) -> (F, F, F, F, F, F, F) {
    let t783 = 0.0 < t772;
    let t2014 = t786 * t695;
    let t2015 = t2014 * t1060;
    let t2016 = t1775 * t2015;
    let t2019 = t785 * t785;
    let t2020 = 1.0 / t2019;
    let t2021 = t657 * t2020;
    let t2023 = piecewise3(t783, t1990, -t1990);
    (t2014, t2015, t2016, t2019, t2020, t2021, t2023)
}
