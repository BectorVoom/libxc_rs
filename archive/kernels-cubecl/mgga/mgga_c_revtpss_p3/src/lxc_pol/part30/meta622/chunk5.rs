//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2142/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2142<F: Float>(t1558: F, t231: F, t25286: F, t25317: F, t25322: F, t25344: F, t25383: F, t25416: F, t27182: F, t27199: F, t27207: F, t2723: F, t4487: F, t7070: F, t7076: F, t886: F, t92922: F, t92925: F, t92930: F, t92935: F, t98897: F, t98907: F, t98911: F, t98918: F, t98920: F, t98922: F) -> F {
    let t98932 = -t98897 - F::cast_from(0.52041769129231196772e1_f64) * t7070 * t25317 * t27182 * t886 + F::cast_from(0.8673628188205199462e0_f64) * t25383 * t27207 + F::cast_from(0.26341796731742046394e1_f64) * t25322 * t4487 + t98907 + F::cast_from(0.8673628188205199462e0_f64) * t27199 * t25344 - t98911 - F::cast_from(0.19514881078765566038e-1_f64) * t92922 - F::cast_from(0.10975748638225852664e-1_f64) * t92925 + F::cast_from(0.10975748638225852664e-1_f64) * t92930 + F::cast_from(0.13009920719177044025e-2_f64) * t92935 + t98918 + F::cast_from(0.65049603595885220126e-3_f64) * t98920 - F::cast_from(0.8673628188205199462e0_f64) * t7070 * t25416 * t98922 * t2723 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t25286 * t1558 * t231;
    t98932
}
