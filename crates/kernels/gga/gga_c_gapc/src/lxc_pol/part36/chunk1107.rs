//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1107/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1107<F: Float>(t33536: F, t33541: F, t33547: F, t33552: F, t33555: F, t33558: F, t33561: F, t33563: F, t33565: F, t33567: F, t33570: F, t33614: F, t33617: F, t33621: F, t33625: F, t33628: F, t33631: F, t33634: F, t33637: F, t33641: F, t33645: F, t33648: F) -> (F, F) {
    let t37798 = 0.88394205998751600033e-7 * t33536 - 0.1076175548412181713e-6 * t33541 + 0.10016653645505750616e-4 * t33547 - 0.17809610181709224597e-4 * t33552 - 0.12141398358188788626e-5 * t33555 + 0.21587406280859666178e-5 * t33558 - 0.12817159869818982005e-5 * t33561 - 0.12817159869818982005e-5 * t33563 + 0.25301106770833333335e-5 * t33565 + 0.10984838052999936404e-3 * t33567 + 0.12141398358188788626e-5 * t33570;
    let t37823 = -0.13937148427849636339e-3 * t33614 + 0.19336232562226912507e-7 * t33617 - 0.2051637995368585198e-8 * t33621 - 0.40441273275208837532e-5 * t33625 - 0.40441273275208837532e-5 * t33628 - 0.9275345110817126956e-4 * t33631 + 0.69504740211613770836e-3 * t33634 + 0.13900948042322754167e-2 * t33637 - 0.9275345110817126956e-4 * t33641 - 0.43284943850479925794e-3 * t33645 + 0.13900948042322754167e-2 * t33648;
    (t37798, t37823)
}
