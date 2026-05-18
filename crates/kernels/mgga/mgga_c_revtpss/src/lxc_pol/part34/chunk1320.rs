//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1320/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1320<F: Float>(t1955: F, t22964: F, t108188: F, t1882: F, t2030: F, t22971: F, t25924: F, t27837: F, t30021: F, t30055: F, t30071: F, t543: F, t6895: F, t6918: F, t7279: F, t7295: F, t7296: F, t7301: F, t7910: F, t7930: F, t94602: F, t94608: F, t97792: F, t97795: F, t97800: F, t97810: F, t97815: F) -> F {
    let t114485 = t1955 * t22964;
    let t114513 = -F::new(0.4336814094102599731e0) * t114485 * t2030 + F::new(0.52041769129231196772e1) * t27837 * t30021 + F::new(0.13010442282307799193e1) * t7295 * t7301 * t30055 * t1882 * t543 - F::new(0.78062653693846795158e1) * t7295 * t25924 * t7910 * t6895 - F::new(0.13010442282307799193e1) * t30071 * t7930 - F::new(0.86736281882051994623e-1) * t108188 + t94602 + F::new(0.26020884564615598386e1) * t7295 * t7296 * t7910 * t6918 + F::new(0.21951497276451705329e-1) * t97792 + F::new(0.19514881078765566038e-2) * t97795 - F::new(0.68549505033305214441e-2) * t97800 - t94608 + F::new(0.34697458558045176417e-2) * t97810 + F::new(0.13709901006661042888e-1) * t97815 + F::new(0.39512695097613069591e1) * t7279 * t22971;
    t114513
}
