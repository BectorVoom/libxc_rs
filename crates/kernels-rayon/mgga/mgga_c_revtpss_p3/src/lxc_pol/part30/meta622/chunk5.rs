//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2142/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2142(t1558: f64, t231: f64, t25286: f64, t25317: f64, t25322: f64, t25344: f64, t25383: f64, t25416: f64, t27182: f64, t27199: f64, t27207: f64, t2723: f64, t4487: f64, t7070: f64, t7076: f64, t886: f64, t92922: f64, t92925: f64, t92930: f64, t92935: f64, t98897: f64, t98907: f64, t98911: f64, t98918: f64, t98920: f64, t98922: f64) -> f64 {
    let t98932 = -t98897 - 0.52041769129231196772e1_f64 * t7070 * t25317 * t27182 * t886 + 0.8673628188205199462e0_f64 * t25383 * t27207 + 0.26341796731742046394e1_f64 * t25322 * t4487 + t98907 + 0.8673628188205199462e0_f64 * t27199 * t25344 - t98911 - 0.19514881078765566038e-1_f64 * t92922 - 0.10975748638225852664e-1_f64 * t92925 + 0.10975748638225852664e-1_f64 * t92930 + 0.13009920719177044025e-2_f64 * t92935 + t98918 + 0.65049603595885220126e-3_f64 * t98920 - 0.8673628188205199462e0_f64 * t7070 * t25416 * t98922 * t2723 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t25286 * t1558 * t231;
    t98932
}
