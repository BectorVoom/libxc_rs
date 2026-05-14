//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1124/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1124<F: Float>(t34905: F, t34907: F, t34909: F, t34911: F, t34914: F, t34918: F, t34921: F, t34926: F, t34929: F, t34934: F, t34936: F, t34940: F, t34942: F, t34946: F, t34949: F, t34951: F, t34954: F, t34956: F, t34958: F, t34960: F, t34962: F, t34965: F) -> (F, F) {
    let t38334 = -0.98332751566569010433e-8 * t34905 + 0.13111033542209201391e-7 * t34907 - 0.33735894097222222226e-5 * t34909 + 0.29518907335069444448e-5 * t34911 + 0.29518907335069444448e-5 * t34914 + 0.1769305705790386642e-5 * t34918 + 0.89870333323222014474e-6 * t34921 - 0.46667986834490740745e-3 * t34926 + 0.54024296947982093732e-5 * t34929 - 0.43830203546981228371e-6 * t34934 + 0.27012148473991046866e-5 * t34936;
    let t38346 = 0.32018399991170720886e-6 * t34940 - 0.81036445421973140598e-5 * t34942 + 0.35358819668152923728e-7 * t34946 - 0.3623181683912940217e-6 * t34949 - 0.10120768229166666668e-4 * t34951 - 0.19676021349741883234e-7 * t34954 - 0.53038229502229385592e-6 * t34956 - 0.53038229502229385592e-6 * t34958 - 0.26519114751114692796e-6 * t34960 - 0.3623181683912940217e-6 * t34962 - 0.90040494913303489552e-7 * t34965;
    (t38334, t38346)
}
