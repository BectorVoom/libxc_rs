//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1178/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1178<F: Float>(t33978: F, t33980: F, t33983: F, t33988: F, t33991: F, t33969: F, t36773: F, t36774: F, t36775: F, t36777: F, t36778: F, t33998: F, t34001: F, t34019: F, t34023: F, t34028: F) -> (F, F, F, F, F, F) {
    let t36779 = 0.47797568285906756368e-9 * t33978;
    let t36780 = 0.11594181388521408695e-4 * t33980;
    let t36781 = 0.27312896163375289353e-9 * t33983;
    let t36782 = 0.49755503537412447748e-6 * t33988;
    let t36783 = 0.18310351929594268994e-5 * t33991;
    let t36784 = -t36773 - t36774 + t36775 - 0.25301106770833333336e-5 * t33969 + t36777 - t36778 + t36779 + t36780 - t36781 - t36782 - t36783;
    let t36788 = 0.71158605186385727883e-8 * t33998;
    let t36789 = 0.13493923611111111112e-4 * t34001;
    let t36793 = 0.94685814672924837674e-4 * t34019;
    let t36794 = 0.41030519691600762993e-3 * t34023;
    let t36795 = 0.89759162297375602412e-9 * t34028;
    (t36784, t36788, t36789, t36793, t36794, t36795)
}
