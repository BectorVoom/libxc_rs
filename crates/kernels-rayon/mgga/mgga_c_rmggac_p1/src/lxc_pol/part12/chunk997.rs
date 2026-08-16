//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 997/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk997(t1652: f64, t664: f64, t2079: f64, t262: f64, t570: f64, t830: f64, t2067: f64, t2353: f64, t26531: f64, t118: f64, t321: f64, t352: f64, t35848: f64, t38948: f64, t40940: f64, t40991: f64, t40993: f64, t41001: f64, t41004: f64, t41006: f64, t4669: f64, t5148: f64, t5266: f64, t866: f64, t8940: f64, t8975: f64) -> (f64, f64) {
    let t41015 = t664 * t1652;
    let t41021 = t2079 * t262 * t830 * t570;
    let t41024 = t26531 * t2067 * t2353;
    let t41026 = -0.23948483403727617128e0_f64 * t35848 - 0.8980681276397856423e-1_f64 * t40991 + 0.17961362552795712846e0_f64 * t40993 - 0.35922725105591425692e0_f64 * t4669 * t40940 * t321 - 0.40911992481368012592e0_f64 * t41001 - 0.81823984962736025184e-1_f64 * t41004 + 0.23948483403727617128e0_f64 * t5266 * t41006 * t352 - 0.11974241701863808564e0_f64 * t5148 * t8975 * t866 - 0.39914139006212695214e-1_f64 * t118 * t38948 + 0.23948483403727617128e0_f64 * t8940 * t41015 * t352 + 0.33335697577410973224e-1_f64 * t41021 - 0.20455996240684006296e-1_f64 * t41024;
    (t41015, t41026)
}
