//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1299/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1299<F: Float>(t33487: F, t33492: F, t33495: F, t33501: F, t33505: F, t33507: F, t33510: F, t33513: F, t33518: F, t33528: F, t33532: F, t33536: F, t33541: F, t33547: F, t33552: F, t33555: F, t33558: F, t33561: F, t33563: F, t33565: F, t33567: F, t33570: F) -> (F, F) {
    let t37786 = -F::new(0.2748593934505475288e-5) * t33487 - F::new(0.49163213094075520838e-7) * t33492 + F::new(0.26681141802169376784e-7) * t33495 - F::new(0.52388299421926781621e-9) * t33501 + F::new(0.49163213094075520838e-8) * t33505 + F::new(0.13526544953274976811e-4) * t33507 - F::new(0.12670134934408760308e-4) * t33510 - F::new(0.99041358770707472872e-5) * t33513 + F::new(0.19336232562226912507e-7) * t33518 + F::new(0.56397344973161828145e-8) * t33528 - F::new(0.11584123368602295139e-4) * t33532;
    let t37798 = F::new(0.88394205998751600033e-7) * t33536 - F::new(0.1076175548412181713e-6) * t33541 + F::new(0.10016653645505750616e-4) * t33547 - F::new(0.17809610181709224597e-4) * t33552 - F::new(0.12141398358188788626e-5) * t33555 + F::new(0.21587406280859666178e-5) * t33558 - F::new(0.12817159869818982005e-5) * t33561 - F::new(0.12817159869818982005e-5) * t33563 + F::new(0.25301106770833333335e-5) * t33565 + F::new(0.10984838052999936404e-3) * t33567 + F::new(0.12141398358188788626e-5) * t33570;
    (t37786, t37798)
}
