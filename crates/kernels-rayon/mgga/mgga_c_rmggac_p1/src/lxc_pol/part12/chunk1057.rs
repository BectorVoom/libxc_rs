//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1057/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1057(t41914: f64, t7720: f64, t236: f64, t495: f64, t7230: f64, t7248: f64, t9216: f64, t7244: f64, t9153: f64, t36701: f64, t36715: f64, t36718: f64, t36735: f64, t41414: f64, t41883: f64, t41885: f64, t41887: f64, t41891: f64, t41893: f64, t41895: f64, t41897: f64, t41902: f64, t41905: f64, t41906: f64, t884: f64) -> f64 {
    let t41915 = t7720 * t41914;
    let t41920 = t7230 * t7248 * t236 * t9216 * t495;
    let t41922 = t7244 * t9153;
    let t41924 = -t36701 + t41883 + t41885 + 0.13637330827122670864e-1_f64 * t41887 - 0.54549323308490683458e-1_f64 * t36715 - 0.85129199786595678796e-5_f64 * t41891 - 0.42564599893297839398e-5_f64 * t41893 + 0.1064114997332445985e-4_f64 * t41895 + 0.25538759935978703638e-4_f64 * t41897 + 0.42564599893297839398e-5_f64 * t41902 + t41905 - 0.10227998120342003148e-1_f64 * t41906 - 0.40650199722100037752e-3_f64 * t36718 + 0.59871208509319042821e-1_f64 * t884 * t41414 + 0.19863479950205658386e-4_f64 * t36735 + 0.17025839957319135759e-4_f64 * t41915 + 0.31923449919973379548e-4_f64 * t41920 + 0.59590439850616975156e-4_f64 * t41922;
    t41924
}
