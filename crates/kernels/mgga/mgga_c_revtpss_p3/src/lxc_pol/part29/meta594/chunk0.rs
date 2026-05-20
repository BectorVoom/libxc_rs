//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1989/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1989<F: Float>(t102573: F, t13739: F, t1904: F, t2027: F, t2028: F, t25930: F, t26304: F, t27868: F, t28911: F, t28918: F, t48020: F, t49380: F, t545: F, t5774: F, t7295: F, t7296: F, t7506: F, t7511: F, t94705: F, t94823: F, t96512: F, t96567: F, t96570: F, t96577: F, t96584: F, t96588: F, t96591: F, t97871: F, t98062: F) -> F {
    let t102700 = F::cast_from(0.13170898365871023197e1_f64) * t7511 * t13739 - F::cast_from(0.17347256376410398924e1_f64) * t27868 * t28911 * t48020 + F::cast_from(0.12851425765524037203e-1_f64) * t96567 + F::cast_from(0.10975748638225852664e-1_f64) * t96570 - F::cast_from(0.19514881078765566038e-1_f64) * t96577 - t96584 + F::cast_from(0.26020884564615598386e1_f64) * t94823 * t26304 * t98062 + F::cast_from(0.17347256376410398924e1_f64) * t25930 * t28911 * t97871 - F::cast_from(0.65854491829355115987e0_f64) * t96512 * t1904 - F::cast_from(0.17347256376410398924e1_f64) * t94705 * t28918 + F::cast_from(0.25702851531048074406e-1_f64) * t96588 + F::cast_from(0.17347256376410398924e1_f64) * t7295 * t7296 * t7506 * t5774 + F::cast_from(0.4336814094102599731e0_f64) * t27868 * t26304 * t49380 - F::cast_from(0.4336814094102599731e0_f64) * t2027 * t2028 * t545 * t102573 + t96591;
    t102700
}
