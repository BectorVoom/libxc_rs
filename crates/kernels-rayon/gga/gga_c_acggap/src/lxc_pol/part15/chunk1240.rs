//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1240/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1240(t31105: f64, t35273: f64, t35278: f64, t35279: f64, t37475: f64, t39840: f64, t39842: f64, t39844: f64, t39846: f64, t39848: f64, t39852: f64, t39856: f64, t39860: f64, t39862: f64, t39867: f64, t39869: f64, t39871: f64) -> f64 {
    let t41832 = 0.51448821741683684368e-2_f64 * t35273 + 0.68598428988911579156e-2_f64 * t39840 + 0.51448821741683684366e-2_f64 * t39842 - 0.34299214494455789578e-2_f64 * t39844 + 0.51448821741683684367e-2_f64 * t39846 - 0.77173232612525526552e-2_f64 * t39848 - 0.37737710747524982482e-2_f64 * t39852 + 0.75475421495049964964e-2_f64 * t39856 - 0.18868855373762491241e-2_f64 * t39860 + 0.17149607247227894789e-2_f64 * t39862 - t35278 - t35279 + 0.37737710747524982481e-2_f64 * t31105 + 0.27439371595564631662e-1_f64 * t39867 + 0.20579528696673473747e-1_f64 * t39869 - 0.51448821741683684367e-2_f64 * t39871 - t37475;
    t41832
}
