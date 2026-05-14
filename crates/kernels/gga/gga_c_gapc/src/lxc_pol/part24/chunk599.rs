//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 599/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk599<F: Float>(t1616: F, t3859: F, t3666: F, t3671: F, t3676: F, t3681: F, t3685: F, t3689: F, t3692: F, t3704: F, t3710: F, t3715: F, t3719: F) -> (F, F) {
    let t3861 = 2.0 * t1616 * t3859;
    let t3873 = 0.80966145833333333339e-4 * t3666 - 0.69504740211613770836e-3 * t3671 - 0.50603841145833333338e-5 * t3676 + 0.43440462632258606772e-4 * t3681 - 0.4637672555408563478e-4 * t3685 - 0.13506074236995523433e-5 * t3689 + 0.11594181388521408695e-4 * t3692 - 0.98332751566569010433e-8 * t3704 + 0.42206481990611010728e-7 * t3710 + 0.13259557375557346398e-6 * t3715 - 0.18115908419564701085e-6 * t3719;
    (t3861, t3873)
}
