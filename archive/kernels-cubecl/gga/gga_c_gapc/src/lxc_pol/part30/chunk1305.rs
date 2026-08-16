//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1305/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1305<F: Float>(t33801: F, t33803: F, t33808: F, t33810: F, t33812: F, t33815: F, t33818: F, t33820: F, t33823: F, t33825: F, t33828: F, t33831: F, t33834: F, t33836: F, t33838: F, t33840: F, t33842: F, t33847: F, t33850: F, t33852: F, t33855: F, t33857: F) -> (F, F) {
    let t37888 = F::cast_from(0.80189736504692130024e-6_f64) * t33801 - F::cast_from(0.42205124476153752644e-7_f64) * t33803 - F::cast_from(0.44197102999375800016e-7_f64) * t33808 - F::cast_from(0.11003142262108589692e-5_f64) * t33810 + F::cast_from(0.8096354166666666667e-4_f64) * t33812 + F::cast_from(0.11584123368602295139e-4_f64) * t33815 - F::cast_from(0.10136107947527008247e-2_f64) * t33818 - F::cast_from(0.69504740211613770836e-3_f64) * t33820 + F::cast_from(0.11584123368602295139e-4_f64) * t33823 - F::cast_from(0.10136107947527008247e-2_f64) * t33825 - F::cast_from(0.69504740211613770836e-3_f64) * t33828;
    let t37900 = -F::cast_from(0.13900948042322754167e-2_f64) * t33831 + F::cast_from(0.4891547309027777778e-4_f64) * t33834 - F::cast_from(0.9275345110817126956e-4_f64) * t33836 - F::cast_from(0.3623181683912940217e-6_f64) * t33838 + F::cast_from(0.24282796716377577252e-5_f64) * t33840 - F::cast_from(0.43174812561719332356e-5_f64) * t33842 + F::cast_from(0.36954560225358884233e-5_f64) * t33847 - F::cast_from(0.67528199161846004231e-6_f64) * t33850 + F::cast_from(0.11196599426508536004e-6_f64) * t33852 + F::cast_from(0.40441273275208837532e-5_f64) * t33855 - F::cast_from(0.2318836277704281739e-4_f64) * t33857;
    (t37888, t37900)
}
