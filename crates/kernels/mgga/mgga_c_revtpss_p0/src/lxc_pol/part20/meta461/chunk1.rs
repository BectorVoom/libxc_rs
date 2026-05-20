//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1754/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1754<F: Float>(t2496: F, t9551: F, t4038: F, t9372: F, t1317: F, t9428: F, t3853: F, t3857: F, t40076: F, t40079: F, t47131: F, t47134: F, t47136: F, t47138: F, t47140: F, t47142: F, t47144: F) -> (F, F, F, F, F) {
    let t47145 = t9551 * t2496;
    let t47146 = F::cast_from(0.10389515463408878255e3_f64) * t47145;
    let t47147 = t4038 * t9372;
    let t47148 = F::cast_from(0.4101607543286562663e4_f64) * t47147;
    let t47149 = t1317 * t9428;
    let t47150 = F::new(48.0) * t47149;
    let t47152 = F::new(120.0) * t3857 * t3853;
    let t47153 = t47131 + t47134 - t47136 - t47138 - t47140 + t47142 + t47144 + t40076 - t40079 - t47146 - t47148 + t47150 + t47152;
    (t47146, t47148, t47150, t47152, t47153)
}
