//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 681/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk681<F: Float>(t1504: F, t2880: F, t8589: F, t8557: F, t8560: F, t8564: F, t8568: F, t8572: F, t8575: F, t8579: F, t8581: F, t8583: F, t8586: F, t8326: F, t8366: F, t8405: F, t8439: F, t8481: F, t8519: F, t8555: F) -> (F,) {
    let t8590 = t2880 * t1504;
    let t8591 = t8589 * t8590;
    let t8593 = 0.43449121406768801912e-4 * t8557 + 0.21724560703384400956e-4 * t8560 + 0.12672660410307567224e-4 * t8564 - 0.43449121406768801912e-4 * t8568 + 0.12672660410307567224e-4 * t8572 - 0.12360406057797588768e-3 * t8575 - 0.43449121406768801912e-4 * t8579 + 0.27517776890953574544e-3 * t8581 + 0.86596512803768376033e-4 * t8583 - 0.10427789137624512459e-2 * t8586 + 0.20855578275249024918e-2 * t8591;
    let t8596 = t8326 + t8366 + t8405 + t8439 + t8481 + t8519 + t8555 + t8593;
    (t8596,)
}
