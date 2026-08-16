//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 927/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk927(t7244: f64, t8437: f64, t7255: f64, t9159: f64, t1614: f64, t1970: f64, t1971: f64, t209: f64, t476: f64, t511: f64, t39927: f64, t39932: f64, t39934: f64, t39940: f64, t39946: f64, t39951: f64, t39954: f64, t39956: f64, t39964: f64, t39966: f64, t39968: f64, t39971: f64, t39975: f64, t4048: f64, t4905: f64, t739: f64, t8800: f64, t884: f64) -> f64 {
    let t39977 = t7244 * t8437;
    let t39978 = 0.19863479950205658386e-4_f64 * t39977;
    let t39979 = t7255 * t9159;
    let t39985 = t1970 * t1971 * t511 * t1614 * t476 * t209;
    let t39987 = -t39927 + 0.85129199786595678796e-5_f64 * t39932 - 0.25538759935978703638e-4_f64 * t39934 - 0.25538759935978703638e-4_f64 * t39940 + 0.42564599893297839398e-5_f64 * t39946 - 0.31923449919973379548e-4_f64 * t39951 - 0.68186654135613354322e-2_f64 * t39954 - 0.8980681276397856423e-1_f64 * t39956 + 0.23948483403727617128e0_f64 * t739 * t8800 * t4048 - 0.23948483403727617128e0_f64 * t884 * t8800 * t4905 - 0.76616279807936110914e-4_f64 * t39964 - 0.25538759935978703638e-4_f64 * t39966 - 0.42564599893297839398e-5_f64 * t39968 + t39971 - 0.85129199786595678796e-5_f64 * t39975 - t39978 + 0.25538759935978703638e-4_f64 * t39979 + 0.25538759935978703638e-4_f64 * t39985;
    t39987
}
