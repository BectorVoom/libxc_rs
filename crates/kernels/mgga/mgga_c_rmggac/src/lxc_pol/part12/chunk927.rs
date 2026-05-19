//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 927/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk927<F: Float>(t7244: F, t8437: F, t7255: F, t9159: F, t1614: F, t1970: F, t1971: F, t209: F, t476: F, t511: F, t39927: F, t39932: F, t39934: F, t39940: F, t39946: F, t39951: F, t39954: F, t39956: F, t39964: F, t39966: F, t39968: F, t39971: F, t39975: F, t4048: F, t4905: F, t739: F, t8800: F, t884: F) -> F {
    let t39977 = t7244 * t8437;
    let t39978 = F::cast_from(0.19863479950205658386e-4_f64) * t39977;
    let t39979 = t7255 * t9159;
    let t39985 = t1970 * t1971 * t511 * t1614 * t476 * t209;
    let t39987 = -t39927 + F::cast_from(0.85129199786595678796e-5_f64) * t39932 - F::cast_from(0.25538759935978703638e-4_f64) * t39934 - F::cast_from(0.25538759935978703638e-4_f64) * t39940 + F::cast_from(0.42564599893297839398e-5_f64) * t39946 - F::cast_from(0.31923449919973379548e-4_f64) * t39951 - F::cast_from(0.68186654135613354322e-2_f64) * t39954 - F::cast_from(0.8980681276397856423e-1_f64) * t39956 + F::cast_from(0.23948483403727617128e0_f64) * t739 * t8800 * t4048 - F::cast_from(0.23948483403727617128e0_f64) * t884 * t8800 * t4905 - F::cast_from(0.76616279807936110914e-4_f64) * t39964 - F::cast_from(0.25538759935978703638e-4_f64) * t39966 - F::cast_from(0.42564599893297839398e-5_f64) * t39968 + t39971 - F::cast_from(0.85129199786595678796e-5_f64) * t39975 - t39978 + F::cast_from(0.25538759935978703638e-4_f64) * t39979 + F::cast_from(0.25538759935978703638e-4_f64) * t39985;
    t39987
}
