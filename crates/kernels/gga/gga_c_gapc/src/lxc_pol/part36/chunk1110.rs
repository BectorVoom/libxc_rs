//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1110/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1110<F: Float>(t33831: F, t33834: F, t33836: F, t33838: F, t33840: F, t33842: F, t33847: F, t33850: F, t33852: F, t33855: F, t33857: F, t33893: F, t33897: F, t33899: F, t33902: F, t33904: F, t33908: F, t33911: F, t33914: F, t33917: F, t33920: F, t33923: F) -> (F, F) {
    let t37900 = -0.13900948042322754167e-2 * t33831 + 0.4891547309027777778e-4 * t33834 - 0.9275345110817126956e-4 * t33836 - 0.3623181683912940217e-6 * t33838 + 0.24282796716377577252e-5 * t33840 - 0.43174812561719332356e-5 * t33842 + 0.36954560225358884233e-5 * t33847 - 0.67528199161846004231e-6 * t33850 + 0.11196599426508536004e-6 * t33852 + 0.40441273275208837532e-5 * t33855 - 0.2318836277704281739e-4 * t33857;
    let t37925 = 0.49196596498842592596e-6 * t33893 - 0.11672999538449102343e-7 * t33897 - 0.42205124476153752644e-7 * t33899 + 0.5497187869010950576e-6 * t33902 - 0.73305000233261025931e-6 * t33904 + 0.26987847222222222224e-4 * t33908 - 0.40481770833333333336e-3 * t33911 + 0.24581606547037760419e-8 * t33914 - 0.94854674673349911132e-9 * t33917 + 0.1011909669415296852e-6 * t33920 + 0.13900948042322754167e-2 * t33923;
    (t37900, t37925)
}
