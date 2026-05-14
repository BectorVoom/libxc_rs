//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1122/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1122<F: Float>(t34550: F, t34553: F, t34555: F, t34557: F, t34560: F, t34563: F, t34565: F, t34567: F, t34570: F, t34573: F, t34576: F, t34605: F, t34608: F, t34611: F, t34613: F, t34615: F, t34617: F, t34619: F, t34622: F, t34625: F, t34630: F, t34633: F) -> (F, F) {
    let t38193 = 0.18115908419564701085e-6 * t34550 + 0.27012148473991046866e-5 * t34553 + 0.27012148473991046866e-5 * t34555 + 0.13506074236995523433e-5 * t34557 + 0.21135226489492151266e-6 * t34560 - 0.23346749993561983979e-7 * t34563 - 0.26519114751114692796e-6 * t34565 - 0.13259557375557346398e-6 * t34567 + 0.2748593934505475288e-5 * t34570 - 0.13506074236995523433e-5 * t34573 - 0.1800809898266069791e-6 * t34576;
    let t38219 = -0.3623181683912940217e-6 * t34605 + 0.7246363367825880434e-6 * t34608 + 0.2023566393031464771e-7 * t34611 - 0.11003142262108589692e-5 * t34613 + 0.8096614583333333334e-3 * t34615 + 0.16193229166666666668e-3 * t34617 - 0.2318836277704281739e-4 * t34619 - 0.69504740211613770836e-3 * t34622 - 0.61781991299212240743e-4 * t34625 + 0.38673709012042260328e-7 * t34630 - 0.86880925264517213544e-4 * t34633;
    (t38193, t38219)
}
