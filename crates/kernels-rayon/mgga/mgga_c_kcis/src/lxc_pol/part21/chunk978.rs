//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 978/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk978(t14775: f64, t382: f64, t14595: f64, t3338: f64, t3337: f64, t1795: f64, t3225: f64, t3466: f64, t3436: f64, t5025: f64, t3439: f64, t14110: f64, t5077: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14776 = t382 * t14775;
    let t14778 = t3338 * t14595;
    let t14779 = t3337 * t14778;
    let t14781 = t1795 * t3225;
    let t14782 = t14781 * sigma0;
    let t14783 = t14782 * t3466;
    let t14785 = t5025 * t3436;
    let t14786 = t14785 * t3439;
    let t14788 = t5077 * t14110;
    (t14776, t14778, t14779, t14781, t14783, t14786, t14788)
}
