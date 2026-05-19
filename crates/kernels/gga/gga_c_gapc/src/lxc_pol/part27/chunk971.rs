//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 971/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk971<F: Float>(t11784: F, t3330: F, t10058: F, t3784: F, t11728: F, t11731: F, t11734: F, t11737: F, t11739: F, t11743: F, t11746: F, t11750: F, t11756: F, t11762: F, t11765: F, t11767: F, t11770: F, t11773: F, t11776: F, t11779: F, t11782: F) -> F {
    let t11785 = t11784 * t3330;
    let t11787 = t3784 * t10058;
    let t11789 = F::cast_from(0.10860115658064651693e-4_f64) * t11728 + F::cast_from(0.10860115658064651693e-4_f64) * t11731 - F::cast_from(0.11594181388521408695e-4_f64) * t11734 + F::cast_from(0.61454016367594401047e-8_f64) * t11737 + F::cast_from(0.81088863580216065975e-3_f64) * t11739 - F::cast_from(0.10860115658064651693e-4_f64) * t11743 - F::cast_from(0.28431716307092827285e-6_f64) * t11746 + F::cast_from(0.11594181388521408695e-4_f64) * t11750 - F::cast_from(0.1264887086769121065e-7_f64) * t11756 + F::cast_from(0.61144341362847222225e-5_f64) * t11762 - F::cast_from(0.17376185052903442709e-3_f64) * t11765 - F::cast_from(0.17376185052903442709e-3_f64) * t11767 + F::cast_from(0.71141006005012433352e-8_f64) * t11770 + F::cast_from(0.63252766927083333336e-6_f64) * t11773 + F::cast_from(0.63252766927083333336e-6_f64) * t11776 + F::cast_from(0.84540905957968605066e-6_f64) * t11779 + F::cast_from(0.12380169846338434109e-5_f64) * t11782 + F::cast_from(0.52756405595192190805e-8_f64) * t11785 + F::cast_from(0.52756405595192190805e-8_f64) * t11787;
    t11789
}
