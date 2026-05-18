//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1306/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1306<F: Float>(t33893: F, t33897: F, t33899: F, t33902: F, t33904: F, t33908: F, t33911: F, t33914: F, t33917: F, t33920: F, t33923: F, t33928: F, t33930: F, t33932: F, t33935: F, t33937: F, t33939: F, t33941: F, t33943: F, t33946: F, t33949: F, t33952: F) -> (F, F) {
    let t37925 = F::new(0.49196596498842592596e-6) * t33893 - F::new(0.11672999538449102343e-7) * t33897 - F::new(0.42205124476153752644e-7) * t33899 + F::new(0.5497187869010950576e-6) * t33902 - F::new(0.73305000233261025931e-6) * t33904 + F::new(0.26987847222222222224e-4) * t33908 - F::new(0.40481770833333333336e-3) * t33911 + F::new(0.24581606547037760419e-8) * t33914 - F::new(0.94854674673349911132e-9) * t33917 + F::new(0.1011909669415296852e-6) * t33920 + F::new(0.13900948042322754167e-2) * t33923;
    let t37938 = F::new(0.45018799441230669486e-7) * t33928 - F::new(0.2023819338830593704e-6) * t33930 + F::new(0.22745373045674261828e-4) * t33932 + F::new(0.5060221354166666667e-5) * t33935 + F::new(0.45018799441230669486e-7) * t33937 + F::new(0.45018799441230669488e-6) * t33939 + F::new(0.66295654499063700024e-7) * t33941 + F::new(0.13259130899812740005e-6) * t33943 + F::new(0.66295654499063700024e-7) * t33946 - F::new(0.27826035332451380868e-3) * t33949 - F::new(0.13900948042322754167e-2) * t33952;
    (t37925, t37938)
}
