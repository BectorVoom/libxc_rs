//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 973/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk973<F: Float>(t40498: F, t40527: F, t40547: F, t40563: F, t47586: F, t47587: F, t47616: F, t47617: F, t47618: F, t47622: F, t47626: F, t47628: F, t47371: F, t1044: F, t10691: F, t12596: F, t12814: F, t12869: F, t1620: F, t1621: F, t186: F, t198: F, t25208: F, t2607: F, t2615: F, t30889: F, t31133: F, t31267: F, t3390: F, t3410: F, t3414: F, t3456: F, t3473: F, t3488: F, t3553: F, t40855: F, t4927: F, t5218: F, t561: F, t639: F) -> (F, F, F, F, F, F, F) {
    let t47629 = 32.0 / 15.0 * t40498;
    let t47630 = 64.0 / 45.0 * t40527;
    let t47631 = 16.0 / 45.0 * t40547;
    let t47632 = 16.0 / 15.0 * t40563;
    let t47633 = t47586 - t47587 + t47616 - t47617 - t47618 + t47622 - t47626 - t47628 + t47629 + t47630 - t47631 - t47632;
    let t47638 = -12.0 * t47371;
    let t47672 = 4.0 / 15.0 * t561 * t186 * t198 * t47638 - 8.0 / 45.0 * t30889 + 8.0 / 5.0 * t3488 * t3456 - 8.0 / 5.0 * t1620 * t1621 * t10691 * t3553 - 16.0 / 15.0 * t1620 * t1621 * t2607 * t12869 + 16.0 / 15.0 * t639 * t1621 * t40855 * t1044 + 32.0 / 15.0 * t2615 * t12814 + 16.0 / 15.0 * t639 * t4927 * t3473 * t3390 - 64.0 / 15.0 * t25208 * t12596 - 32.0 / 15.0 * t5218 * t31267 * t3414 - 32.0 / 15.0 * t5218 * t31133 * t3410;
    (t47629, t47630, t47631, t47632, t47633, t47638, t47672)
}
