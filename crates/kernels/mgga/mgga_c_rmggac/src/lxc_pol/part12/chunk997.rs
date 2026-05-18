//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 997/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk997<F: Float>(t1652: F, t664: F, t2079: F, t262: F, t570: F, t830: F, t2067: F, t2353: F, t26531: F, t118: F, t321: F, t352: F, t35848: F, t38948: F, t40940: F, t40991: F, t40993: F, t41001: F, t41004: F, t41006: F, t4669: F, t5148: F, t5266: F, t866: F, t8940: F, t8975: F) -> (F, F) {
    let t41015 = t664 * t1652;
    let t41021 = t2079 * t262 * t830 * t570;
    let t41024 = t26531 * t2067 * t2353;
    let t41026 = -F::new(0.23948483403727617128e0) * t35848 - F::new(0.8980681276397856423e-1) * t40991 + F::new(0.17961362552795712846e0) * t40993 - F::new(0.35922725105591425692e0) * t4669 * t40940 * t321 - F::new(0.40911992481368012592e0) * t41001 - F::new(0.81823984962736025184e-1) * t41004 + F::new(0.23948483403727617128e0) * t5266 * t41006 * t352 - F::new(0.11974241701863808564e0) * t5148 * t8975 * t866 - F::new(0.39914139006212695214e-1) * t118 * t38948 + F::new(0.23948483403727617128e0) * t8940 * t41015 * t352 + F::new(0.33335697577410973224e-1) * t41021 - F::new(0.20455996240684006296e-1) * t41024;
    (t41015, t41026)
}
