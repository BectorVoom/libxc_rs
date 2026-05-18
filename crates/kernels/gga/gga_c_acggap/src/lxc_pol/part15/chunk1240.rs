//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1240/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1240<F: Float>(t31105: F, t35273: F, t35278: F, t35279: F, t37475: F, t39840: F, t39842: F, t39844: F, t39846: F, t39848: F, t39852: F, t39856: F, t39860: F, t39862: F, t39867: F, t39869: F, t39871: F) -> F {
    let t41832 = F::new(0.51448821741683684368e-2) * t35273 + F::new(0.68598428988911579156e-2) * t39840 + F::new(0.51448821741683684366e-2) * t39842 - F::new(0.34299214494455789578e-2) * t39844 + F::new(0.51448821741683684367e-2) * t39846 - F::new(0.77173232612525526552e-2) * t39848 - F::new(0.37737710747524982482e-2) * t39852 + F::new(0.75475421495049964964e-2) * t39856 - F::new(0.18868855373762491241e-2) * t39860 + F::new(0.17149607247227894789e-2) * t39862 - t35278 - t35279 + F::new(0.37737710747524982481e-2) * t31105 + F::new(0.27439371595564631662e-1) * t39867 + F::new(0.20579528696673473747e-1) * t39869 - F::new(0.51448821741683684367e-2) * t39871 - t37475;
    t41832
}
