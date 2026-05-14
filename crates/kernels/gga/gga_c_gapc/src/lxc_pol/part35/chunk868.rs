//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 868/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk868<F: Float>(t11784: F, t3330: F, t10058: F, t3784: F, t11728: F, t11731: F, t11734: F, t11737: F, t11739: F, t11743: F, t11746: F, t11750: F, t11756: F, t11762: F, t11765: F, t11767: F, t11770: F, t11773: F, t11776: F, t11779: F, t11782: F) -> (F,) {
    let t11785 = t11784 * t3330;
    let t11787 = t3784 * t10058;
    let t11789 = 0.10860115658064651693e-4 * t11728 + 0.10860115658064651693e-4 * t11731 - 0.11594181388521408695e-4 * t11734 + 0.61454016367594401047e-8 * t11737 + 0.81088863580216065975e-3 * t11739 - 0.10860115658064651693e-4 * t11743 - 0.28431716307092827285e-6 * t11746 + 0.11594181388521408695e-4 * t11750 - 0.1264887086769121065e-7 * t11756 + 0.61144341362847222225e-5 * t11762 - 0.17376185052903442709e-3 * t11765 - 0.17376185052903442709e-3 * t11767 + 0.71141006005012433352e-8 * t11770 + 0.63252766927083333336e-6 * t11773 + 0.63252766927083333336e-6 * t11776 + 0.84540905957968605066e-6 * t11779 + 0.12380169846338434109e-5 * t11782 + 0.52756405595192190805e-8 * t11785 + 0.52756405595192190805e-8 * t11787;
    (t11789,)
}
