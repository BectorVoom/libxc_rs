//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1214/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1214(t13853: f64, t169: f64, t21204: f64, t4043: f64, t519: f64, t11430: f64, t3060: f64, t8716: f64, t34995: f64, t35001: f64, t35003: f64, t35005: f64, t35007: f64, t35010: f64, t35013: f64, t35016: f64, t35019: f64) -> f64 {
    let t35024 = t169 * t21204 * t4043 * t519 * t13853;
    let t35027 = t3060 * t11430 * t8716;
    let t35029 = 0.10793703140429833089e-5_f64 * t34995 - 0.12187980608940473897e-4_f64 * t35001 + 0.4637672555408563478e-4_f64 * t35003 - 0.4637672555408563478e-4_f64 * t35005 + 0.49522272202316919254e-5_f64 * t35007 - 0.22510123728325872388e-6_f64 * t35010 - 0.38647271295071362318e-6_f64 * t35013 - 0.6629778687778673199e-7_f64 * t35016 + 0.40022999988963401106e-8_f64 * t35019 - 0.24877751768706223874e-6_f64 * t35024 + 0.30775559784820528656e-8_f64 * t35027;
    t35029
}
