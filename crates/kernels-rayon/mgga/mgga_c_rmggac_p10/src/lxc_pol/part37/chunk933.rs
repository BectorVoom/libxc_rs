//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 933/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk933(t76891: f64, t68514: f64, t68517: f64, t74191: f64, t74193: f64, t74195: f64, t74203: f64, t15502: f64, t495: f64, t515: f64, t7230: f64, t7231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76892 = 0.68186654135613354322e-2_f64 * t76891;
    let t76893 = 0.81300399444200075499e-3_f64 * t68514;
    let t76894 = 0.81300399444200075499e-3_f64 * t68517;
    let t76896 = 0.10227998120342003148e-1_f64 * t74191;
    let t76897 = 0.25650144397517585626e-6_f64 * t74193;
    let t76898 = 0.25650144397517585626e-6_f64 * t74195;
    let t76904 = 0.23268647941669485538e-4_f64 * t74203;
    let t76912 = t7230 * t7231 * t515 * t15502 * t495;
    (t76892, t76893, t76894, t76896, t76897, t76898, t76904, t76912)
}
