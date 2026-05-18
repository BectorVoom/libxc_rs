//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1057/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1057<F: Float>(t41914: F, t7720: F, t236: F, t495: F, t7230: F, t7248: F, t9216: F, t7244: F, t9153: F, t36701: F, t36715: F, t36718: F, t36735: F, t41414: F, t41883: F, t41885: F, t41887: F, t41891: F, t41893: F, t41895: F, t41897: F, t41902: F, t41905: F, t41906: F, t884: F) -> F {
    let t41915 = t7720 * t41914;
    let t41920 = t7230 * t7248 * t236 * t9216 * t495;
    let t41922 = t7244 * t9153;
    let t41924 = -t36701 + t41883 + t41885 + F::new(0.13637330827122670864e-1) * t41887 - F::new(0.54549323308490683458e-1) * t36715 - F::new(0.85129199786595678796e-5) * t41891 - F::new(0.42564599893297839398e-5) * t41893 + F::new(0.1064114997332445985e-4) * t41895 + F::new(0.25538759935978703638e-4) * t41897 + F::new(0.42564599893297839398e-5) * t41902 + t41905 - F::new(0.10227998120342003148e-1) * t41906 - F::new(0.40650199722100037752e-3) * t36718 + F::new(0.59871208509319042821e-1) * t884 * t41414 + F::new(0.19863479950205658386e-4) * t36735 + F::new(0.17025839957319135759e-4) * t41915 + F::new(0.31923449919973379548e-4) * t41920 + F::new(0.59590439850616975156e-4) * t41922;
    t41924
}
