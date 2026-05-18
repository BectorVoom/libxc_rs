//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 683/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk683<F: Float>(t515: F, t9843: F, t1971: F, t7230: F, t2310: F, t8571: F, t2320: F, t9222: F, t1763: F, t7703: F, t1356: F, t1737: F, t665: F) -> (F, F, F, F, F, F, F) {
    let t9844 = t515 * t9843;
    let t9845 = t1971 * t9844;
    let t9846 = t7230 * t9845;
    let t9847 = F::new(0.1064114997332445985e-4) * t9846;
    let t9848 = t8571 * t2310;
    let t9849 = F::new(0.85129199786595678796e-5) * t9848;
    let t9850 = t9222 * t2320;
    let t9851 = F::new(0.1064114997332445985e-4) * t9850;
    let t9852 = t7703 * t1763;
    let t9853 = t1356 * t9852;
    let t9854 = F::new(0.11974241701863808564e0) * t9853;
    let t9855 = t665 * t1737;
    (t9845, t9847, t9849, t9851, t9852, t9854, t9855)
}
