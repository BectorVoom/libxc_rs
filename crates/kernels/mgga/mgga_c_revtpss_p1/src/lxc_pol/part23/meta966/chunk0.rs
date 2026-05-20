//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3264/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3264<F: Float>(t48262: F, t47011: F, t48269: F, t22789: F, t72: F, t757: F, t73476: F, t39783: F, t39786: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t47059: F, t48261: F, t48266: F, t48268: F, t48271: F) -> (F, F, F, F, F, F) {
    let t85908 = F::cast_from(0.17544670867903938621e1_f64) * t48262;
    let t85909 = F::cast_from(0.56968947174242584612e-3_f64) * t47011;
    let t85910 = F::cast_from(0.15584273195113317383e3_f64) * t48269;
    let t85912 = t22789 * t72 * t757;
    let t85913 = F::cast_from(0.18311447306006545054e-3_f64) * t85912;
    let t85914 = F::new(3.0) * t73476;
    let t85915 = t48261 - t85908 - t39783 - t39786 - t39791 - t39795 - t85909 + t48266 + t48268 - t85910 - t85913 + t48271 + t39799 + t47059 + t85914 + t39807 - t39813;
    (t85908, t85909, t85910, t85913, t85914, t85915)
}
