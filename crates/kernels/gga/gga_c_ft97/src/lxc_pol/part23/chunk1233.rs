//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1233/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1233<F: Float>(t123675: F, t27661: F, t108508: F, t108518: F, t108519: F, t108525: F, t108526: F, t1091: F, t1096: F, t122765: F, t123006: F, t123582: F, t123607: F, t123612: F, t123615: F, t123619: F, t123650: F, t123672: F, t1417: F, t1701: F, t17987: F, t18007: F, t18015: F, t18084: F, t2035: F, t24276: F, t24278: F, t24346: F, t27487: F, t27500: F, t27527: F, t27529: F, t27605: F, t27658: F, t31515: F, t3700: F, t3817: F, t505: F, t6023: F, t6027: F, t6979: F, t709: F, t96424: F, t96600: F, t96602: F, t96716: F) -> (F, F) {
    let t123676 = t123675 * t27661;
    let t123679 = 0.51690243689028715487e-4 * t27527 * t6023 * t123582 + 0.24511020009968991684e-5 * t123607 * t27605 * t27529 - 0.3959138103817207526e-3 * t123612 - 0.98910212891072794758e-5 * t96600 * t96602 * t123615 + 0.3520097786805302698e-5 * t108525 * t24278 * t1096 * t123619 + 0.29693535778629056444e-3 * t108518 * t24278 * t1096 * t123006 * t505 - 0.29693535778629056444e-3 * t96716 * t24278 * t1096 * t122765 * t505 + 0.22227677429409423704e-2 * t1417 * t1701 * t6027 * t18084 + 0.1054015240332537869e-3 * t17987 * t2035 * t6979 * t3817 + 0.23254900946437792e-1 * t24346 * t18007 + 0.46509801892875584e-2 * t27487 * t18015 - 0.89591295428265718861e-3 * t17987 * t2035 * t31515 * t709 + 0.29693535778629056444e-3 * t108518 * t108519 * t123650 + 0.3520097786805302698e-5 * t108525 * t108526 * t123650 - 0.29693535778629056444e-3 * t96716 * t24278 * t1096 * t1091 * t709 - 0.29693535778629056444e-4 * t24276 * t96424 * t123615 - 0.29693535778629056444e-3 * t24276 * t24278 * t1096 * t3700 - 0.29693535778629056444e-3 * t96716 * t108508 * t123650 - 0.6809984893827160494e-1 * t27500 * t123672 + 0.17659850543899795697e-2 * t27658 * t123676;
    (t123676, t123679)
}
