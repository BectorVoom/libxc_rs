//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1417/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1417<F: Float>(t24: F, t27911: F, t27916: F, t27930: F, t27939: F, t27987: F, t27989: F, t27991: F, t27993: F, t27995: F, t27998: F, t28001: F, t28005: F, t28010: F, t28012: F, t22148: F, t10375: F, t10384: F, t1263: F, t1265: F, t2467: F, t2471: F, t26879: F, t28576: F, t28578: F, t28583: F, t28589: F, t28593: F, t28603: F, t3289: F, t3293: F, t3940: F, t3944: F, t422: F, t423: F, t8577: F, t8587: F, t960: F, t962: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t28604 = t27911 - t27987 + t27989 + t27916 - t27991 + t27993 + t27995 + t27998 - t27930 - t28001 - t28005 - t27939 - t28010 + t28012;
    let t28620 = piecewise3(t90, 0.0, -t22148);
    let t28624 = piecewise3(t332, 0.0, (t26879 + t28576 + t28578 + t28583 + t28589 + t28593 + t28603 + t28604) * t423 / 2.0 + t10375 * t962 + t3940 * t2471 / 2.0 + t8577 * t1265 + 2.0 * t3289 * t3293 + t1263 * t8587 + t2467 * t3944 / 2.0 + t960 * t10384 + t422 * t28620 / 2.0);
    (t28624,)
}
